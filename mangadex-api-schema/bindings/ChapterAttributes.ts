// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Language } from "./Language";
import type { MangaDexDateTime } from "./MangaDexDateTime";

export interface ChapterAttributes { title: string, volume: string | null, chapter: string | null, pages: number, translatedLanguage: Language, uploader?: string, externalUrl: string, version: number, createdAt: MangaDexDateTime, updatedAt: MangaDexDateTime | null, publishAt: MangaDexDateTime, readableAt: MangaDexDateTime, }