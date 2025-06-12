# \WalletsApi

All URIs are relative to *http://..*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_wallets_wallet_name**](WalletsApi.md#delete_wallets_wallet_name) | **DELETE** /wallets/{wallet_name} | Delete your wallet file (can be recovered with your mnemonic)
[**get_wallets**](WalletsApi.md#get_wallets) | **GET** /wallets | List available wallets
[**get_wallets_wallet_name**](WalletsApi.md#get_wallets_wallet_name) | **GET** /wallets/{wallet_name} | Get wallet's status
[**get_wallets_wallet_name_addresses**](WalletsApi.md#get_wallets_wallet_name_addresses) | **GET** /wallets/{wallet_name}/addresses | List all your wallet's addresses
[**get_wallets_wallet_name_addresses_address**](WalletsApi.md#get_wallets_wallet_name_addresses_address) | **GET** /wallets/{wallet_name}/addresses/{address} | Get address' info
[**get_wallets_wallet_name_balances**](WalletsApi.md#get_wallets_wallet_name_balances) | **GET** /wallets/{wallet_name}/balances | Get your total balance
[**post_wallets**](WalletsApi.md#post_wallets) | **POST** /wallets | Create a new wallet
[**post_wallets_wallet_name_change_active_address**](WalletsApi.md#post_wallets_wallet_name_change_active_address) | **POST** /wallets/{wallet_name}/change-active-address | Choose the active address
[**post_wallets_wallet_name_derive_next_address**](WalletsApi.md#post_wallets_wallet_name_derive_next_address) | **POST** /wallets/{wallet_name}/derive-next-address | Derive your next address
[**post_wallets_wallet_name_lock**](WalletsApi.md#post_wallets_wallet_name_lock) | **POST** /wallets/{wallet_name}/lock | Lock your wallet
[**post_wallets_wallet_name_reveal_mnemonic**](WalletsApi.md#post_wallets_wallet_name_reveal_mnemonic) | **POST** /wallets/{wallet_name}/reveal-mnemonic | Reveal your mnemonic. !!! use it with caution !!!
[**post_wallets_wallet_name_sign**](WalletsApi.md#post_wallets_wallet_name_sign) | **POST** /wallets/{wallet_name}/sign | Sign the given data and return back the signature
[**post_wallets_wallet_name_sweep_active_address**](WalletsApi.md#post_wallets_wallet_name_sweep_active_address) | **POST** /wallets/{wallet_name}/sweep-active-address | Transfer all unlocked ALPH from the active address to another address
[**post_wallets_wallet_name_sweep_all_addresses**](WalletsApi.md#post_wallets_wallet_name_sweep_all_addresses) | **POST** /wallets/{wallet_name}/sweep-all-addresses | Transfer unlocked ALPH from all addresses (including all mining addresses if applicable) to another address
[**post_wallets_wallet_name_transfer**](WalletsApi.md#post_wallets_wallet_name_transfer) | **POST** /wallets/{wallet_name}/transfer | Transfer ALPH from the active address
[**post_wallets_wallet_name_unlock**](WalletsApi.md#post_wallets_wallet_name_unlock) | **POST** /wallets/{wallet_name}/unlock | Unlock your wallet
[**put_wallets**](WalletsApi.md#put_wallets) | **PUT** /wallets | Restore a wallet from your mnemonic



## delete_wallets_wallet_name

> delete_wallets_wallet_name(wallet_name, password)
Delete your wallet file (can be recovered with your mnemonic)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**password** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallets

> Vec<models::WalletStatus> get_wallets()
List available wallets

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WalletStatus>**](WalletStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallets_wallet_name

> models::WalletStatus get_wallets_wallet_name(wallet_name)
Get wallet's status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |

### Return type

[**models::WalletStatus**](WalletStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallets_wallet_name_addresses

> models::Addresses get_wallets_wallet_name_addresses(wallet_name)
List all your wallet's addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |

### Return type

[**models::Addresses**](Addresses.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallets_wallet_name_addresses_address

> models::AddressInfo get_wallets_wallet_name_addresses_address(wallet_name, address)
Get address' info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**address** | **String** |  | [required] |

### Return type

[**models::AddressInfo**](AddressInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallets_wallet_name_balances

> models::Balances get_wallets_wallet_name_balances(wallet_name)
Get your total balance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |

### Return type

[**models::Balances**](Balances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets

> models::WalletCreationResult post_wallets(wallet_creation)
Create a new wallet

A new wallet will be created and respond with a mnemonic. Make sure to keep that mnemonic safely as it will allows you to recover your wallet. Default mnemonic size is 24, (options: 12, 15, 18, 21, 24).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_creation** | [**WalletCreation**](WalletCreation.md) |  | [required] |

### Return type

[**models::WalletCreationResult**](WalletCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_change_active_address

> post_wallets_wallet_name_change_active_address(wallet_name, change_active_address)
Choose the active address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**change_active_address** | [**ChangeActiveAddress**](ChangeActiveAddress.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_derive_next_address

> models::AddressInfo post_wallets_wallet_name_derive_next_address(wallet_name, group)
Derive your next address

Cannot be called from a miner wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**group** | Option<**i32**> |  |  |

### Return type

[**models::AddressInfo**](AddressInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_lock

> post_wallets_wallet_name_lock(wallet_name)
Lock your wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_reveal_mnemonic

> models::RevealMnemonicResult post_wallets_wallet_name_reveal_mnemonic(wallet_name, reveal_mnemonic)
Reveal your mnemonic. !!! use it with caution !!!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**reveal_mnemonic** | [**RevealMnemonic**](RevealMnemonic.md) |  | [required] |

### Return type

[**models::RevealMnemonicResult**](RevealMnemonicResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_sign

> models::SignResult post_wallets_wallet_name_sign(wallet_name, sign)
Sign the given data and return back the signature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**sign** | [**Sign**](Sign.md) |  | [required] |

### Return type

[**models::SignResult**](SignResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_sweep_active_address

> models::TransferResults post_wallets_wallet_name_sweep_active_address(wallet_name, sweep)
Transfer all unlocked ALPH from the active address to another address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**sweep** | [**Sweep**](Sweep.md) |  | [required] |

### Return type

[**models::TransferResults**](TransferResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_sweep_all_addresses

> models::TransferResults post_wallets_wallet_name_sweep_all_addresses(wallet_name, sweep)
Transfer unlocked ALPH from all addresses (including all mining addresses if applicable) to another address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**sweep** | [**Sweep**](Sweep.md) |  | [required] |

### Return type

[**models::TransferResults**](TransferResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_transfer

> models::TransferResult post_wallets_wallet_name_transfer(wallet_name, transfer)
Transfer ALPH from the active address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**transfer** | [**Transfer**](Transfer.md) | Format 1: `1000000000000000000`  Format 2: `x.y ALPH`, where `1 ALPH = 1000000000000000000  Field fromPublicKeyType can be  `default` or `bip340-schnorr` | [required] |

### Return type

[**models::TransferResult**](TransferResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_wallets_wallet_name_unlock

> post_wallets_wallet_name_unlock(wallet_name, wallet_unlock)
Unlock your wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_name** | **String** |  | [required] |
**wallet_unlock** | [**WalletUnlock**](WalletUnlock.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_wallets

> models::WalletRestoreResult put_wallets(wallet_restore)
Restore a wallet from your mnemonic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet_restore** | [**WalletRestore**](WalletRestore.md) |  | [required] |

### Return type

[**models::WalletRestoreResult**](WalletRestoreResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

