# \BlockflowApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_blockflow_blocks**](BlockflowApi.md#get_blockflow_blocks) | **GET** /blockflow/blocks | List blocks on the given time interval
[**get_blockflow_blocks_block_hash**](BlockflowApi.md#get_blockflow_blocks_block_hash) | **GET** /blockflow/blocks/{block_hash} | Get a block with hash
[**get_blockflow_blocks_with_events**](BlockflowApi.md#get_blockflow_blocks_with_events) | **GET** /blockflow/blocks-with-events | List blocks with events on the given time interval
[**get_blockflow_blocks_with_events_block_hash**](BlockflowApi.md#get_blockflow_blocks_with_events_block_hash) | **GET** /blockflow/blocks-with-events/{block_hash} | Get a block and events with hash
[**get_blockflow_chain_info**](BlockflowApi.md#get_blockflow_chain_info) | **GET** /blockflow/chain-info | Get infos about the chain from the given groups
[**get_blockflow_hashes**](BlockflowApi.md#get_blockflow_hashes) | **GET** /blockflow/hashes | Get all block's hashes at given height for given groups
[**get_blockflow_headers_block_hash**](BlockflowApi.md#get_blockflow_headers_block_hash) | **GET** /blockflow/headers/{block_hash} | Get block header
[**get_blockflow_is_block_in_main_chain**](BlockflowApi.md#get_blockflow_is_block_in_main_chain) | **GET** /blockflow/is-block-in-main-chain | Check if the block is in main chain
[**get_blockflow_main_chain_block_by_ghost_uncle_ghost_uncle_hash**](BlockflowApi.md#get_blockflow_main_chain_block_by_ghost_uncle_ghost_uncle_hash) | **GET** /blockflow/main-chain-block-by-ghost-uncle/{ghost_uncle_hash} | Get a mainchain block by ghost uncle hash
[**get_blockflow_raw_blocks_block_hash**](BlockflowApi.md#get_blockflow_raw_blocks_block_hash) | **GET** /blockflow/raw-blocks/{block_hash} | Get raw block in hex format
[**get_blockflow_rich_blocks**](BlockflowApi.md#get_blockflow_rich_blocks) | **GET** /blockflow/rich-blocks | Given a time interval, list blocks containing events and transactions with enriched input information when node indexes are enabled.
[**get_blockflow_rich_blocks_block_hash**](BlockflowApi.md#get_blockflow_rich_blocks_block_hash) | **GET** /blockflow/rich-blocks/{block_hash} | Get a block containing events and transactions with enriched input information when node indexes are enabled.



## get_blockflow_blocks

> models::BlocksPerTimeStampRange get_blockflow_blocks(from_ts, to_ts)
List blocks on the given time interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_ts** | **i64** |  | [required] |
**to_ts** | Option<**i64**> |  |  |

### Return type

[**models::BlocksPerTimeStampRange**](BlocksPerTimeStampRange.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_blocks_block_hash

> models::BlockEntry get_blockflow_blocks_block_hash(block_hash)
Get a block with hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_hash** | **String** |  | [required] |

### Return type

[**models::BlockEntry**](BlockEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_blocks_with_events

> models::BlocksAndEventsPerTimeStampRange get_blockflow_blocks_with_events(from_ts, to_ts)
List blocks with events on the given time interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_ts** | **i64** |  | [required] |
**to_ts** | Option<**i64**> |  |  |

### Return type

[**models::BlocksAndEventsPerTimeStampRange**](BlocksAndEventsPerTimeStampRange.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_blocks_with_events_block_hash

> models::BlockAndEvents get_blockflow_blocks_with_events_block_hash(block_hash)
Get a block and events with hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_hash** | **String** |  | [required] |

### Return type

[**models::BlockAndEvents**](BlockAndEvents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_chain_info

> models::ChainInfo get_blockflow_chain_info(from_group, to_group)
Get infos about the chain from the given groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_group** | **i32** |  | [required] |
**to_group** | **i32** |  | [required] |

### Return type

[**models::ChainInfo**](ChainInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_hashes

> models::HashesAtHeight get_blockflow_hashes(from_group, to_group, height)
Get all block's hashes at given height for given groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_group** | **i32** |  | [required] |
**to_group** | **i32** |  | [required] |
**height** | **i32** |  | [required] |

### Return type

[**models::HashesAtHeight**](HashesAtHeight.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_headers_block_hash

> models::BlockHeaderEntry get_blockflow_headers_block_hash(block_hash)
Get block header

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_hash** | **String** |  | [required] |

### Return type

[**models::BlockHeaderEntry**](BlockHeaderEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_is_block_in_main_chain

> bool get_blockflow_is_block_in_main_chain(block_hash)
Check if the block is in main chain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_hash** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_main_chain_block_by_ghost_uncle_ghost_uncle_hash

> models::BlockEntry get_blockflow_main_chain_block_by_ghost_uncle_ghost_uncle_hash(ghost_uncle_hash)
Get a mainchain block by ghost uncle hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ghost_uncle_hash** | **String** |  | [required] |

### Return type

[**models::BlockEntry**](BlockEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_raw_blocks_block_hash

> models::RawBlock get_blockflow_raw_blocks_block_hash(block_hash)
Get raw block in hex format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_hash** | **String** |  | [required] |

### Return type

[**models::RawBlock**](RawBlock.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_rich_blocks

> models::RichBlocksAndEventsPerTimeStampRange get_blockflow_rich_blocks(from_ts, to_ts)
Given a time interval, list blocks containing events and transactions with enriched input information when node indexes are enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_ts** | **i64** |  | [required] |
**to_ts** | Option<**i64**> |  |  |

### Return type

[**models::RichBlocksAndEventsPerTimeStampRange**](RichBlocksAndEventsPerTimeStampRange.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockflow_rich_blocks_block_hash

> models::RichBlockAndEvents get_blockflow_rich_blocks_block_hash(block_hash)
Get a block containing events and transactions with enriched input information when node indexes are enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_hash** | **String** |  | [required] |

### Return type

[**models::RichBlockAndEvents**](RichBlockAndEvents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

