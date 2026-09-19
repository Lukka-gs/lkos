import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import ts from "typescript";
const allowed = {
  shared: ["shared"],
  core: ["core", "shared"],
  settings: ["settings", "shared"],
  platform: ["platform", "shared", "settings"],
  ui: ["ui", "shared", "settings"],
  gadgets: ["shared"],
  tools: ["shared"],
  actions: ["shared"],
  triggers: ["shared"],
  workspaces: ["shared"],
  shell: [
    "core",
    "shell",
    "ui",
    "platform",
    "shared",
    "settings",
    "gadgets",
    "tools",
    "actions",
    "triggers",
    "workspaces",
  ],
};
let violations = 0;
for (const file of readdirSync("src", { recursive: true }).filter((f) =>
  f.endsWith(".ts"),
)) {
  const layer = file.split(path.sep)[0];
  const ast = ts.createSourceFile(
    file,
    readFileSync(path.join("src", file), "utf8"),
    ts.ScriptTarget.Latest,
    true,
  );
  function visit(node) {
    const specifier =
      ts.isImportDeclaration(node) || ts.isExportDeclaration(node)
        ? node.moduleSpecifier
        : ts.isCallExpression(node) &&
            node.expression.kind === ts.SyntaxKind.ImportKeyword
          ? node.arguments[0]
          : undefined;
    if (specifier && ts.isStringLiteral(specifier)) {
      const spec = specifier.text;
      const targetParts = path
        .normalize(path.join(path.dirname(file), spec))
        .split(path.sep);
      const target = spec.startsWith(".") ? targetParts[0] : spec;
      const sourceParts = file.split(path.sep);
      const ownModule =
        ["gadgets", "tools"].includes(layer) &&
        sourceParts.length > 2 &&
        target === layer &&
        targetParts[1] === sourceParts[1];
      const external = !spec.startsWith(".");
      if (
        allowed[layer] &&
        (external
          ? spec !== "vitest" &&
            !(layer === "platform" && spec.startsWith("@tauri-apps/api/"))
          : !ownModule &&
            !allowed[layer].includes(target) &&
            !(target === layer && !["gadgets", "tools"].includes(layer)))
      ) {
        console.error(`${file}: forbidden dependency ${spec}`);
        violations++;
      }
    }
    ts.forEachChild(node, visit);
  }
  visit(ast);
}
if (violations) process.exit(1);
console.log("Module dependency boundaries passed");
