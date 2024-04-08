import * as vscode from 'coc.nvim';
import { resolve } from 'path';
import { get_selected_range } from './utils';
import { readFileSync } from 'fs';
import init, * as dioxus from 'dioxus-ext';

export async function activate(context: vscode.ExtensionContext) {
	// Load the wasm from the file system
  const wasmSourceCode = readFileSync(resolve(context.extensionPath, "./pkg/dioxus_ext_bg.wasm"));

	// Wait for the initialization to finish
	// This is using the byte buffer directly which won't go through the "fetch" machinery
	//
	// For whatever reason, wasm-bindgen generates `fetch` when we don't want it to
	// VSCode doesn't have a `fetch` implementation, but we don't really care about polyfilling it
	await init(wasmSourceCode);

	// Todo:
	// I want a paste-handler that translates HTML to RSX whenever HTML is pasted into an Rsx block
	// Or, a little tooltip that pops up and asks if you want to translate the HTML to RSX
	context.subscriptions.push(
		vscode.commands.registerCommand('extension.htmlToDioxusRsx', () => translate(false)),
		vscode.commands.registerCommand('extension.htmlToDioxusComponent', () => translate(true)),
		vscode.commands.registerCommand('extension.formatRsx', fmtSelection),
		vscode.commands.registerCommand('extension.formatRsxDocument', formatRsxDocument),
		vscode.workspace.onWillSaveTextDocument(fmtDocumentOnSave)
	);
}

async function translate(component: boolean) {
	// Load the activate editor
	const editor = vscode.window.activeTextEditor;
	if (!editor) return;

	// Get the selected text
	const html = editor.document.textDocument.getText(await get_selected_range());
	if (html.length == 0) {
		vscode.window.showWarningMessage("Please select HTML fragment before invoking this command!");
		return;
	}

	// Translate the HTML to RSX
	const out = dioxus.translate_rsx(html, component);
	if (out.length > 0) {
    editor.document.applyEdits([vscode.TextEdit.replace(await get_selected_range(), out) ]);
	} else {
		vscode.window.showWarningMessage(`Errors occurred while translating, make sure this block of HTML is valid`);
	}
}


function formatRsxDocument() {
	const editor = vscode.window.activeTextEditor;
	if (!editor) return;

	fmtDocument(editor.document.textDocument);
}

async function fmtSelection() {
	const editor = vscode.window.activeTextEditor;
	if (!editor) return;

	if (editor.document.languageId !== "rust") {
		return;
	}

  const selection = await get_selected_range();

	let end_line = selection.end.line;

	// Select full lines of selection
	let selection_range = vscode.Range.create(
    {
      line: selection.start.line, 
      character: 0, 
    },
    {

      line: end_line, 
      character: editor.document.textDocument.lineAt(end_line).range.end.character
    }
	);

	let unformatted = editor.document.textDocument.getText(selection_range);

	if (unformatted.trim().length == 0) {
		vscode.window.showWarningMessage("Please select rsx invoking this command!");
		return;
	}

	// If number of closing braces is lower than opening braces, expand selection to end of initial block
	while ((unformatted.match(/{/g) || []).length > (unformatted.match(/}/g) || []).length && end_line < editor.document.lineCount - 1) {
		end_line += 1;

		selection_range = vscode.Range.create(
      {
        line: selection.start.line, 
        character: 0, 
      },
      {
        line: end_line, 
        character: editor.document.textDocument.lineAt(end_line).range.end.character
      }
		);

		unformatted = editor.document.textDocument.getText(selection_range);
	}

	let tabSize: number;
	if (typeof editor.options.tabSize === 'number') {
		tabSize = editor.options.tabSize;
	} else {
		tabSize = 4;
	}

	const end_above = Math.max(selection.start.line - 1, 0);

	const lines_above = editor.document.textDocument.getText(
		vscode.Range.create(
      {
        line: 0,
        character: 0,
      },
      {
        line: end_above,
        character: editor.document.textDocument.lineAt(end_above).range.end.character
      }
		)
	);

	// Calculate indent for current selection
	const base_indentation = (lines_above.match(/{/g) || []).length - (lines_above.match(/}/g) || []).length - 1;

	try {
		let formatted = dioxus.format_selection(unformatted, !editor.options.insertSpaces, tabSize, base_indentation);
		for(let i = 0; i <= base_indentation; i++) {
			formatted = (editor.options.insertSpaces ? " ".repeat(tabSize) : "\t") + formatted;
		}
		if (formatted.length > 0) {
			editor.document.applyEdits([
				vscode.TextEdit.replace(selection_range, formatted)
      ]);
		}
	} catch (error) {
		vscode.window.showErrorMessage(`Errors occurred while formatting. Make sure you have the most recent Dioxus-CLI installed and you have selected valid rsx with your cursor! \n${error}`);
	}

}

function fmtDocumentOnSave(e: vscode.TextDocumentWillSaveEvent) {
	// check the settings to make sure format on save is configured
	const dioxusConfig = vscode.workspace.getConfiguration('dioxus', e.document).get('formatOnSave');
	const globalConfig = vscode.workspace.getConfiguration('editor', e.document).get('formatOnSave');
	if (
		(dioxusConfig === 'enabled') ||
		(dioxusConfig !== 'disabled' && globalConfig)
	) {
		fmtDocument(e.document);
	}
}

function fmtDocument(document: vscode.TextDocument) {
	try {
		if (document.languageId !== "rust") {
			return;
		}

		const [editor,] = vscode.window.visibleTextEditors.filter(editor => editor.document.uri === document.uri);
		if (!editor) return; // Need an editor to apply text edits.

		const contents = editor.document.textDocument.getText();
		let tabSize: number;
		if (typeof editor.options.tabSize === 'number') {
			tabSize = editor.options.tabSize;
		} else {
			tabSize = 4;
		}
		const formatted = dioxus.format_file(contents, !editor.options.insertSpaces, tabSize);

		// Replace the entire text document
		// Yes, this is a bit heavy handed, but the dioxus side doesn't know the line/col scheme that vscode is using
		if (formatted.length() > 0) {
			editor.document.applyEdits([
        vscode.TextEdit.replace(vscode.Range.create({ character: 0, line: 0 }, { line: document.lineCount, character: 0}), formatted.formatted())
      ]);
		}
	} catch (error) {
		vscode.window.showWarningMessage(`Errors occurred while formatting. Make sure you have the most recent Dioxus-CLI installed! \n${error}`);
	}
}
