/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Serializer,
  array,
  struct,
} from '@metaplex-foundation/umi/serializers';
import { Attribute, AttributeArgs, getAttributeSerializer } from '.';

export type Attributes = { attributeList: Array<Attribute> };

export type AttributesArgs = { attributeList: Array<AttributeArgs> };

export function getAttributesSerializer(): Serializer<
  AttributesArgs,
  Attributes
> {
  return struct<Attributes>(
    [['attributeList', array(getAttributeSerializer())]],
    { description: 'Attributes' }
  ) as Serializer<AttributesArgs, Attributes>;
}
