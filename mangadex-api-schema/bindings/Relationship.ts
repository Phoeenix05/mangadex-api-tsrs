// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { MangaRelation } from "./MangaRelation";
import type { RelatedAttributes } from "./RelatedAttributes";
import type { RelationshipType } from "./RelationshipType";

export interface Relationship { id: string, type: RelationshipType, related?: MangaRelation, attributes?: RelatedAttributes, }