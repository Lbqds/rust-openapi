# \MinersApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_miners_addresses**](MinersApi.md#get_miners_addresses) | **GET** /miners/addresses | List miner's addresses
[**get_wallets_wallet_name_miner_addresses**](MinersApi.md#get_wallets_wallet_name_miner_addresses) | **GET** /wallets/{wallet_name}/miner-addresses | List all miner addresses per group
[**post_miners_cpu_mining**](MinersApi.md#post_miners_cpu_mining) | **POST** /miners/cpu-mining | Execute an action on CPU miner. !!! for test only !!!
[**post_miners_cpu_mining_mine_one_block**](MinersApi.md#post_miners_cpu_mining_mine_one_block) | **POST** /miners/cpu-mining/mine-one-block | Mine a block on CPU miner. !!! for test only !!!
[**post_wallets_wallet_name_derive_next_miner_addresses**](MinersApi.md#post_wallets_wallet_name_derive_next_miner_addresses) | **POST** /wallets/{wallet_name}/derive-next-miner-addresses | Derive your next miner addresses for each group
[**put_miners_addresses**](MinersApi.md#put_miners_addresses) | **PUT** /miners/addresses | Update miner's addresses, but better to use user.conf instead



## get_miners_addresses

> models::MinerAddresses get_miners_addresses()
List miner's addresses

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MinerAddresses**](MinerAddresses.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallets_wallet_name_miner_addresses

> Vec<models::MinerAddressesInfo> get_wallets_wallet_name_miner_addresses(wallet_name)
List all miner addresses per group

This endpoint can only be called if the wallet was created with the `isMiner = true` flag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |

### Return type

[**Vec<models::MinerAddressesInfo>**](MinerAddressesInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_miners_cpu_mining

> bool post_miners_cpu_mining(action)
Execute an action on CPU miner. !!! for test only !!!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_miners_cpu_mining_mine_one_block

> bool post_miners_cpu_mining_mine_one_block(from_group, to_group)
Mine a block on CPU miner. !!! for test only !!!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_group** | **i32** |  | [required] |
**to_group** | **i32** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_derive_next_miner_addresses

> Vec<models::AddressInfo> post_wallets_wallet_name_derive_next_miner_addresses(wallet_name)
Derive your next miner addresses for each group

Your wallet need to have been created with the miner flag set to true

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |

### Return type

[**Vec<models::AddressInfo>**](AddressInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_miners_addresses

> put_miners_addresses(miner_addresses)
Update miner's addresses, but better to use user.conf instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_addresses** | [**MinerAddresses**](MinerAddresses.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

