const assert = require("node:assert/strict");
const fs = require("node:fs");
const Module = require("node:module");
const path = require("node:path");
const ts = require("typescript");

const repoRoot = path.resolve(__dirname, "..");
const originalJsLoader = Module._extensions[".js"];

function installTypeScriptLoader() {
  const compile = (module, filename) => {
    const source = fs.readFileSync(filename, "utf8");
    const output = ts.transpileModule(source, {
      compilerOptions: {
        esModuleInterop: true,
        jsx: ts.JsxEmit.ReactJSX,
        module: ts.ModuleKind.CommonJS,
        target: ts.ScriptTarget.ES2020,
      },
      fileName: filename,
    }).outputText;

    module._compile(output, filename);
  };

  Module._extensions[".ts"] = compile;
  Module._extensions[".tsx"] = compile;
}

function uninstallTypeScriptLoader() {
  delete Module._extensions[".ts"];
  delete Module._extensions[".tsx"];
  Module._extensions[".js"] = originalJsLoader;
}

function findInput(element) {
  if (!element || typeof element !== "object") {
    return null;
  }

  if (element.type === "input") {
    return element;
  }

  const children = element.props?.children;
  const list = Array.isArray(children) ? children : [children];

  for (const child of list) {
    const input = findInput(child);
    if (input) {
      return input;
    }
  }

  return null;
}

function event(value, key) {
  return {
    currentTarget: {
      value: String(value),
    },
    key,
  };
}

function renderSlider(value = 30000) {
  const commits = [];
  const changes = [];
  const { BrightnessSlider } = require(path.join(repoRoot, "src", "components", "BrightnessSlider.tsx"));

  const element = BrightnessSlider({
    disabled: false,
    onChange: (nextValue) => changes.push(nextValue),
    onCommit: (nextValue) => commits.push(nextValue),
    value,
  });
  const input = findInput(element);

  assert.ok(input, "BrightnessSlider should render a range input");
  return { changes, commits, input };
}

function run() {
  installTypeScriptLoader();

  try {
    const focusedOnly = renderSlider();
    focusedOnly.input.props.onBlur?.(event(30000));
    assert.deepEqual(focusedOnly.commits, [], "blur without a slider interaction must not commit brightness");

    const changedOnly = renderSlider();
    changedOnly.input.props.onChange(event(32000));
    assert.deepEqual(changedOnly.changes, [32000], "change should update the staged slider value");
    assert.deepEqual(changedOnly.commits, [], "change alone should not commit until the user confirms the slider");

    const pointerConfirmed = renderSlider();
    pointerConfirmed.input.props.onPointerUp(event(32000));
    assert.deepEqual(pointerConfirmed.commits, [32000], "pointer confirmation should commit brightness");

    const enterConfirmed = renderSlider();
    enterConfirmed.input.props.onKeyUp(event(33000, "Enter"));
    assert.deepEqual(enterConfirmed.commits, [33000], "Enter should commit brightness");

    const spaceConfirmed = renderSlider();
    spaceConfirmed.input.props.onKeyUp(event(34000, " "));
    assert.deepEqual(spaceConfirmed.commits, [34000], "Space should commit brightness");

    const arrowAdjusted = renderSlider();
    arrowAdjusted.input.props.onKeyUp(event(35000, "ArrowRight"));
    assert.deepEqual(arrowAdjusted.commits, [35000], "slider adjustment keys should commit brightness");

    const unrelatedKey = renderSlider();
    unrelatedKey.input.props.onKeyUp(event(36000, "Tab"));
    assert.deepEqual(unrelatedKey.commits, [], "non-adjustment keys should not commit brightness");
  } finally {
    uninstallTypeScriptLoader();
  }
}

run();
