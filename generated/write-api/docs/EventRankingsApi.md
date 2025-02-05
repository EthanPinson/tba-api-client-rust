# \EventRankingsApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_rankings**](EventRankingsApi.md#update_rankings) | **POST** /event/{eventKey}/rankings/update | Update team rankings at an event



## update_rankings

> update_rankings(event_key, update_rankings_request)
Update team rankings at an event

Update rankings at an event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**update_rankings_request** | [**UpdateRankingsRequest**](UpdateRankingsRequest.md) | A dict of `breakdowns` and `rankings | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

