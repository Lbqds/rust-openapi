# \ContractsApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_contracts_address_parent**](ContractsApi.md#get_contracts_address_parent) | **GET** /contracts/{address}/parent | Get parent contract address
[**get_contracts_address_state**](ContractsApi.md#get_contracts_address_state) | **GET** /contracts/{address}/state | Get contract state
[**get_contracts_address_sub_contracts**](ContractsApi.md#get_contracts_address_sub_contracts) | **GET** /contracts/{address}/sub-contracts | Get sub-contract addresses
[**get_contracts_address_sub_contracts_current_count**](ContractsApi.md#get_contracts_address_sub_contracts_current_count) | **GET** /contracts/{address}/sub-contracts/current-count | Get current value of the sub-contracts counter for a contract
[**get_contracts_codehash_code**](ContractsApi.md#get_contracts_codehash_code) | **GET** /contracts/{codeHash}/code | Get contract code by code hash
[**post_contracts_call_contract**](ContractsApi.md#post_contracts_call_contract) | **POST** /contracts/call-contract | Call contract
[**post_contracts_call_tx_script**](ContractsApi.md#post_contracts_call_tx_script) | **POST** /contracts/call-tx-script | Call TxScript
[**post_contracts_compile_contract**](ContractsApi.md#post_contracts_compile_contract) | **POST** /contracts/compile-contract | Compile a smart contract
[**post_contracts_compile_project**](ContractsApi.md#post_contracts_compile_project) | **POST** /contracts/compile-project | Compile a project
[**post_contracts_compile_script**](ContractsApi.md#post_contracts_compile_script) | **POST** /contracts/compile-script | Compile a script
[**post_contracts_multicall_contract**](ContractsApi.md#post_contracts_multicall_contract) | **POST** /contracts/multicall-contract | Multiple call contract
[**post_contracts_test_contract**](ContractsApi.md#post_contracts_test_contract) | **POST** /contracts/test-contract | Test contract
[**post_contracts_unsigned_tx_deploy_contract**](ContractsApi.md#post_contracts_unsigned_tx_deploy_contract) | **POST** /contracts/unsigned-tx/deploy-contract | Build an unsigned contract
[**post_contracts_unsigned_tx_execute_script**](ContractsApi.md#post_contracts_unsigned_tx_execute_script) | **POST** /contracts/unsigned-tx/execute-script | Build an unsigned script



## get_contracts_address_parent

> String get_contracts_address_parent(address)
Get parent contract address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts_address_state

> models::ContractState get_contracts_address_state(address)
Get contract state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**models::ContractState**](ContractState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts_address_sub_contracts

> models::SubContracts get_contracts_address_sub_contracts(address, start, limit)
Get sub-contract addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**start** | **i32** |  | [required] |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::SubContracts**](SubContracts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts_address_sub_contracts_current_count

> i32 get_contracts_address_sub_contracts_current_count(address)
Get current value of the sub-contracts counter for a contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts_codehash_code

> String get_contracts_codehash_code(code_hash)
Get contract code by code hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_hash** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_call_contract

> models::CallContractResult post_contracts_call_contract(call_contract)
Call contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_contract** | [**CallContract**](CallContract.md) |  | [required] |

### Return type

[**models::CallContractResult**](CallContractResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_call_tx_script

> models::CallTxScriptResult post_contracts_call_tx_script(call_tx_script)
Call TxScript

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_tx_script** | [**CallTxScript**](CallTxScript.md) |  | [required] |

### Return type

[**models::CallTxScriptResult**](CallTxScriptResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_compile_contract

> models::CompileContractResult post_contracts_compile_contract(contract)
Compile a smart contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | [**Contract**](Contract.md) |  | [required] |

### Return type

[**models::CompileContractResult**](CompileContractResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_compile_project

> models::CompileProjectResult post_contracts_compile_project(project)
Compile a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::CompileProjectResult**](CompileProjectResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_compile_script

> models::CompileScriptResult post_contracts_compile_script(script)
Compile a script

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script** | [**Script**](Script.md) |  | [required] |

### Return type

[**models::CompileScriptResult**](CompileScriptResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_multicall_contract

> models::MultipleCallContractResult post_contracts_multicall_contract(multiple_call_contract)
Multiple call contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**multiple_call_contract** | [**MultipleCallContract**](MultipleCallContract.md) |  | [required] |

### Return type

[**models::MultipleCallContractResult**](MultipleCallContractResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_test_contract

> models::TestContractResult post_contracts_test_contract(test_contract)
Test contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_contract** | [**TestContract**](TestContract.md) |  | [required] |

### Return type

[**models::TestContractResult**](TestContractResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_unsigned_tx_deploy_contract

> models::BuildDeployContractTxResult post_contracts_unsigned_tx_deploy_contract(build_deploy_contract_tx)
Build an unsigned contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_deploy_contract_tx** | [**BuildDeployContractTx**](BuildDeployContractTx.md) |  | [required] |

### Return type

[**models::BuildDeployContractTxResult**](BuildDeployContractTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contracts_unsigned_tx_execute_script

> models::BuildExecuteScriptTxResult post_contracts_unsigned_tx_execute_script(build_execute_script_tx)
Build an unsigned script

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_execute_script_tx** | [**BuildExecuteScriptTx**](BuildExecuteScriptTx.md) |  | [required] |

### Return type

[**models::BuildExecuteScriptTxResult**](BuildExecuteScriptTxResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

