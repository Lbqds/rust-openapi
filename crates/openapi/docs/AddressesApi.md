# \AddressesApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_addresses_address_balance**](AddressesApi.md#get_addresses_address_balance) | **GET** /addresses/{address}/balance | Get the balance of an address
[**get_addresses_address_group**](AddressesApi.md#get_addresses_address_group) | **GET** /addresses/{address}/group | Get the group of an address
[**get_addresses_address_utxos**](AddressesApi.md#get_addresses_address_utxos) | **GET** /addresses/{address}/utxos | Get the UTXOs of an address



## get_addresses_address_balance

> models::Balance get_addresses_address_balance(address, mempool)
Get the balance of an address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**mempool** | Option<**bool**> |  |  |

### Return type

[**models::Balance**](Balance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addresses_address_group

> models::Group get_addresses_address_group(address)
Get the group of an address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addresses_address_utxos

> models::Utxos get_addresses_address_utxos(address)
Get the UTXOs of an address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**models::Utxos**](UTXOs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

