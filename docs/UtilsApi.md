# \UtilsApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_utils_target_to_hashrate**](UtilsApi.md#post_utils_target_to_hashrate) | **POST** /utils/target-to-hashrate | Convert a target to hashrate
[**post_utils_verify_signature**](UtilsApi.md#post_utils_verify_signature) | **POST** /utils/verify-signature | Verify the SecP256K1 signature of some data
[**put_utils_check_hash_indexing**](UtilsApi.md#put_utils_check_hash_indexing) | **PUT** /utils/check-hash-indexing | Check and repair the indexing of block hashes



## post_utils_target_to_hashrate

> models::Result post_utils_target_to_hashrate(target_to_hashrate)
Convert a target to hashrate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_to_hashrate** | [**TargetToHashrate**](TargetToHashrate.md) |  | [required] |

### Return type

[**models::Result**](Result.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_utils_verify_signature

> bool post_utils_verify_signature(verify_signature)
Verify the SecP256K1 signature of some data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_signature** | [**VerifySignature**](VerifySignature.md) |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_utils_check_hash_indexing

> put_utils_check_hash_indexing()
Check and repair the indexing of block hashes

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

