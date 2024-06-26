// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { SaveToFileOpenFileError } from "./SaveToFileOpenFileError";
import type { SaveToFileWriteError } from "./SaveToFileWriteError";

export type SaveToFileError = { "type": "writing", part: SaveToFileWriteError, } | { "type": "keyDerivation" } | { "type": "createCipher" } | { "type": "openFile", case: SaveToFileOpenFileError, } | { "type": "serialization" } | { "type": "passwordCheck" };