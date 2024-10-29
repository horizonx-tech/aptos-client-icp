# \EventsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events_by_creation_number**](EventsApi.md#get_events_by_creation_number) | **GET** /accounts/{address}/events/{creation_number} | Get events by creation number
[**get_events_by_event_handle**](EventsApi.md#get_events_by_event_handle) | **GET** /accounts/{address}/events/{event_handle}/{field_name} | Get events by event handle



## get_events_by_creation_number

> Vec<models::VersionedEvent> get_events_by_creation_number(address, creation_number, start, limit)
Get events by creation number

Event types are globally identifiable by an account `address` and monotonically increasing `creation_number`, one per event type emitted to the given account. This API returns events corresponding to that that event type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Hex-encoded 32 byte Aptos account, with or without a `0x` prefix, for which events are queried. This refers to the account that events were emitted to, not the account hosting the move module that emits that event type. | [required] |
**creation_number** | **String** | Creation number corresponding to the event stream originating from the given account. | [required] |
**start** | Option<**String**> | Starting sequence number of events.  If unspecified, by default will retrieve the most recent events |  |
**limit** | Option<**i32**> | Max number of events to retrieve.  If unspecified, defaults to default page size |  |

### Return type

[**Vec<models::VersionedEvent>**](VersionedEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_event_handle

> Vec<models::VersionedEvent> get_events_by_event_handle(address, event_handle, field_name, start, limit)
Get events by event handle

This API uses the given account `address`, `eventHandle`, and `fieldName` to build a key that can globally identify an event types. It then uses this key to return events emitted to the given account matching that event type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Hex-encoded 32 byte Aptos account, with or without a `0x` prefix, for which events are queried. This refers to the account that events were emitted to, not the account hosting the move module that emits that event type. | [required] |
**event_handle** | **String** | Name of struct to lookup event handle e.g. `0x1::account::Account` | [required] |
**field_name** | **String** | Name of field to lookup event handle e.g. `withdraw_events` | [required] |
**start** | Option<**String**> | Starting sequence number of events.  If unspecified, by default will retrieve the most recent |  |
**limit** | Option<**i32**> | Max number of events to retrieve.  If unspecified, defaults to default page size |  |

### Return type

[**Vec<models::VersionedEvent>**](VersionedEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bcs

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

