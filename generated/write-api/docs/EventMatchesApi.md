# \EventMatchesApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_matches**](EventMatchesApi.md#delete_matches) | **POST** /event/{eventKey}/matches/delete | Delete matches at an event
[**update_matches**](EventMatchesApi.md#update_matches) | **POST** /event/{eventKey}/matches/update | Update matches at an event



## delete_matches

> delete_matches(event_key, request_body)
Delete matches at an event

Delete one or more matches under an event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**request_body** | [**Vec<String>**](String.md) | List of match keys | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_matches

> update_matches(event_key, r#match)
Update matches at an event

Add/Update one or more matches under an event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**r#match** | [**Vec<models::Match>**](Match.md) | List of matches | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

