# CompileContractResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **String** |  | 
**name** | **String** |  | 
**bytecode** | **String** |  | 
**bytecode_debug_patch** | **String** |  | 
**code_hash** | **String** |  | 
**code_hash_debug** | **String** |  | 
**fields** | [**models::FieldsSig**](FieldsSig.md) |  | 
**functions** | [**Vec<models::FunctionSig>**](FunctionSig.md) |  | 
**constants** | [**Vec<models::Constant>**](Constant.md) |  | 
**enums** | [**Vec<models::Enum>**](Enum.md) |  | 
**events** | [**Vec<models::EventSig>**](EventSig.md) |  | 
**warnings** | **Vec<String>** |  | 
**maps** | Option<[**models::MapsSig**](MapsSig.md)> |  | [optional]
**std_interface_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


