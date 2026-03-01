# \EventWebcastsApi

All URIs are relative to *https://www.thebluealliance.com/api/trusted/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_event_webcasts**](EventWebcastsApi.md#add_event_webcasts) | **PATCH** /event/{eventKey}/webcasts/update | Add webcasts to an event
[**delete_event_webcasts**](EventWebcastsApi.md#delete_event_webcasts) | **DELETE** /event/{eventKey}/webcasts/update | Remove webcasts from an event



## add_event_webcasts

> add_event_webcasts(event_key, webcast_update)
Add webcasts to an event

Append new webcasts to an event's existing webcast list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**webcast_update** | [**WebcastUpdate**](WebcastUpdate.md) | List of webcasts to add | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_event_webcasts

> delete_event_webcasts(event_key, webcast_update)
Remove webcasts from an event

Remove specified webcasts from an event's webcast list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA event key to update. For example, 2017cthar | [required] |
**webcast_update** | [**WebcastUpdate**](WebcastUpdate.md) | List of webcasts to remove | [required] |

### Return type

 (empty response body)

### Authorization

[authId](../README.md#authId), [authSig](../README.md#authSig)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

