import * as vscode from "coc.nvim";
import { TextDecoder, TextEncoder } from "util";

global.TextDecoder = TextDecoder;
global.TextEncoder = TextEncoder;

export async function get_selected_range(): Promise<vscode.Range> {
  const mode = (await vscode.workspace.nvim.call("visualmode")) as string;
  const selection = await vscode.window.getSelectedRange(mode);
  const editor = vscode.window.activeTextEditor;
  return selection &&
    mode === "V" &&
    editor &&
    selection.end.line > selection.start.line
    ? {
        start: selection.start,
        end: {
          line: selection.end.line - 1,
          character: editor.document.textDocument.lineAt(selection.end.line - 1)
            .range.end.character,
        },
      }
    : selection ||
        vscode.Range.create(
          {
            character: 0,
            line: 0,
          },
          {
            character: 0,
            line: 0,
          }
        );
}
