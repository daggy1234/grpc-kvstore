# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: kvstore.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import builder as _builder
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\rkvstore.proto\x12\x07kvstore\"\"\n\x04Item\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t\"\x17\n\x08Response\x12\x0b\n\x03key\x18\x01 \x01(\t\"\x18\n\tGetParams\x12\x0b\n\x03key\x18\x01 \x01(\t\"(\n\x08\x41llItems\x12\x1c\n\x05items\x18\x01 \x03(\x0b\x32\r.kvstore.Item\"\x1a\n\x07Success\x12\x0f\n\x07success\x18\x01 \x01(\x08\"\x10\n\x0e\x41llItemsParams2\xca\x01\n\x0c\x43\x61\x63heService\x12#\n\x03Put\x12\r.kvstore.Item\x1a\r.kvstore.Item\x12(\n\x03Get\x12\x12.kvstore.GetParams\x1a\r.kvstore.Item\x12;\n\x0bGetAllItems\x12\x17.kvstore.AllItemsParams\x1a\x11.kvstore.AllItems0\x01\x12.\n\x06\x44\x65lete\x12\x12.kvstore.GetParams\x1a\x10.kvstore.Successb\x06proto3')

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'kvstore_pb2', globals())
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _ITEM._serialized_start=26
  _ITEM._serialized_end=60
  _RESPONSE._serialized_start=62
  _RESPONSE._serialized_end=85
  _GETPARAMS._serialized_start=87
  _GETPARAMS._serialized_end=111
  _ALLITEMS._serialized_start=113
  _ALLITEMS._serialized_end=153
  _SUCCESS._serialized_start=155
  _SUCCESS._serialized_end=181
  _ALLITEMSPARAMS._serialized_start=183
  _ALLITEMSPARAMS._serialized_end=199
  _CACHESERVICE._serialized_start=202
  _CACHESERVICE._serialized_end=404
# @@protoc_insertion_point(module_scope)