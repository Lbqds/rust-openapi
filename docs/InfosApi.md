# \InfosApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_infos_chain_params**](InfosApi.md#get_infos_chain_params) | **GET** /infos/chain-params | Get key params about your blockchain
[**get_infos_current_difficulty**](InfosApi.md#get_infos_current_difficulty) | **GET** /infos/current-difficulty | Get the average difficulty of the latest blocks from all shards
[**get_infos_current_hashrate**](InfosApi.md#get_infos_current_hashrate) | **GET** /infos/current-hashrate | Get average hashrate from `now - timespan(millis)` to `now`
[**get_infos_discovered_neighbors**](InfosApi.md#get_infos_discovered_neighbors) | **GET** /infos/discovered-neighbors | Get discovered neighbors
[**get_infos_history_hashrate**](InfosApi.md#get_infos_history_hashrate) | **GET** /infos/history-hashrate | Get history average hashrate on the given time interval
[**get_infos_inter_clique_peer_info**](InfosApi.md#get_infos_inter_clique_peer_info) | **GET** /infos/inter-clique-peer-info | Get infos about the inter cliques
[**get_infos_misbehaviors**](InfosApi.md#get_infos_misbehaviors) | **GET** /infos/misbehaviors | Get the misbehaviors of peers
[**get_infos_node**](InfosApi.md#get_infos_node) | **GET** /infos/node | Get info about that node
[**get_infos_self_clique**](InfosApi.md#get_infos_self_clique) | **GET** /infos/self-clique | Get info about your own clique
[**get_infos_unreachable**](InfosApi.md#get_infos_unreachable) | **GET** /infos/unreachable | Get the unreachable brokers
[**get_infos_version**](InfosApi.md#get_infos_version) | **GET** /infos/version | Get version about that node
[**post_infos_discovery**](InfosApi.md#post_infos_discovery) | **POST** /infos/discovery | Set brokers to be unreachable/reachable
[**post_infos_misbehaviors**](InfosApi.md#post_infos_misbehaviors) | **POST** /infos/misbehaviors | Ban/Unban given peers



## get_infos_chain_params

> models::ChainParams get_infos_chain_params()
Get key params about your blockchain

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChainParams**](ChainParams.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_current_difficulty

> models::CurrentDifficulty get_infos_current_difficulty()
Get the average difficulty of the latest blocks from all shards

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CurrentDifficulty**](CurrentDifficulty.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_current_hashrate

> models::HashRateResponse get_infos_current_hashrate(timespan)
Get average hashrate from `now - timespan(millis)` to `now`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timespan** | Option<**i64**> |  |  |

### Return type

[**models::HashRateResponse**](HashRateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_discovered_neighbors

> Vec<models::BrokerInfo> get_infos_discovered_neighbors()
Get discovered neighbors

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BrokerInfo>**](BrokerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_history_hashrate

> models::HashRateResponse get_infos_history_hashrate(from_ts, to_ts)
Get history average hashrate on the given time interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_ts** | **i64** |  | [required] |
**to_ts** | Option<**i64**> |  |  |

### Return type

[**models::HashRateResponse**](HashRateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_inter_clique_peer_info

> Vec<models::InterCliquePeerInfo> get_infos_inter_clique_peer_info()
Get infos about the inter cliques

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::InterCliquePeerInfo>**](InterCliquePeerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_misbehaviors

> Vec<models::PeerMisbehavior> get_infos_misbehaviors()
Get the misbehaviors of peers

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PeerMisbehavior>**](PeerMisbehavior.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_node

> models::NodeInfo get_infos_node()
Get info about that node

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NodeInfo**](NodeInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_self_clique

> models::SelfClique get_infos_self_clique()
Get info about your own clique

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SelfClique**](SelfClique.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_unreachable

> Vec<String> get_infos_unreachable()
Get the unreachable brokers

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_infos_version

> models::NodeVersion get_infos_version()
Get version about that node

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NodeVersion**](NodeVersion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_infos_discovery

> post_infos_discovery(discovery_action)
Set brokers to be unreachable/reachable

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discovery_action** | [**DiscoveryAction**](DiscoveryAction.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_infos_misbehaviors

> post_infos_misbehaviors(misbehavior_action)
Ban/Unban given peers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**misbehavior_action** | [**MisbehaviorAction**](MisbehaviorAction.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

