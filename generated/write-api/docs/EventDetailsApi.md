# \EventDetailsApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_event_info**](EventDetailsApi.md#update_event_info) | **POST** /event/{eventKey}/info/update | Update top-level properties for the event



## update_event_info

> update_event_info(event_key, event_info)
Update top-level properties for the event

An endpoint to overwrite certain event fields. All fields are optional, set only the ones you wish to update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**event_info** | [**EventInfo**](EventInfo.md) | A JSON dictionary, mapping property name to the new value. | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

