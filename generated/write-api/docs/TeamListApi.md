# \TeamListApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_team_list**](TeamListApi.md#update_team_list) | **POST** /event/{eventKey}/team_list/update | Update the list of teams attending an event



## update_team_list

> update_team_list(event_key, request_body)
Update the list of teams attending an event

Replaces the list of teams attending

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**request_body** | [**Vec<String>**](String.md) | List of team keys | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

