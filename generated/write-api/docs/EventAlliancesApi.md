# \EventAlliancesApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_alliances**](EventAlliancesApi.md#update_alliances) | **POST** /event/{eventKey}/alliance_selections/update | Update the alliance selections at an event



## update_alliances

> update_alliances(event_key, request_body)
Update the alliance selections at an event

Overwrite the event's alliance selections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**request_body** | [**Vec<Vec<String>>**](Vec.md) | A list of alliances, in order of alliance number. An alliance is a list of team keys, in order of selection. An alliance can have an arbitrary number of teams (provided that all alliances have the same number of teams). | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

