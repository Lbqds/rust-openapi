# \MultiSignatureApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_multisig_address**](MultiSignatureApi.md#post_multisig_address) | **POST** /multisig/address | Create the multisig address and unlock script
[**post_multisig_build**](MultiSignatureApi.md#post_multisig_build) | **POST** /multisig/build | Build a multisig unsigned transaction
[**post_multisig_submit**](MultiSignatureApi.md#post_multisig_submit) | **POST** /multisig/submit | Submit a multi-signed transaction
[**post_multisig_sweep**](MultiSignatureApi.md#post_multisig_sweep) | **POST** /multisig/sweep | Sweep all unlocked ALPH and token balances of a multisig address to another address



## post_multisig_address

> models::BuildMultisigAddressResult post_multisig_address(build_multisig_address)
Create the multisig address and unlock script

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_multisig_address** | [**BuildMultisigAddress**](BuildMultisigAddress.md) | Format 1: `1000000000000000000`  Format 2: `x.y ALPH`, where `1 ALPH = 1000000000000000000  Field fromPublicKeyType can be  `default` or `bip340-schnorr` | [required] |

### Return type

[**models::BuildMultisigAddressResult**](BuildMultisigAddressResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_multisig_build

> models::BuildTransferTxResult post_multisig_build(build_multisig)
Build a multisig unsigned transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_multisig** | [**BuildMultisig**](BuildMultisig.md) |  | [required] |

### Return type

[**models::BuildTransferTxResult**](BuildTransferTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_multisig_submit

> models::SubmitTxResult post_multisig_submit(submit_multisig)
Submit a multi-signed transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_multisig** | [**SubmitMultisig**](SubmitMultisig.md) |  | [required] |

### Return type

[**models::SubmitTxResult**](SubmitTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_multisig_sweep

> models::BuildSweepAddressTransactionsResult post_multisig_sweep(build_sweep_multisig)
Sweep all unlocked ALPH and token balances of a multisig address to another address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_sweep_multisig** | [**BuildSweepMultisig**](BuildSweepMultisig.md) |  | [required] |

### Return type

[**models::BuildSweepAddressTransactionsResult**](BuildSweepAddressTransactionsResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

