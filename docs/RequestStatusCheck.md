# RequestStatusCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**finished** | Option<**i32**> | The amount of finished images in this request | [optional]
**processing** | Option<**i32**> | The amount of still processing images in this request | [optional]
**waiting** | Option<**i32**> | The amount of images waiting to be picked up by a worker | [optional]
**done** | Option<**bool**> | True when all images in this request are done. Else False. | [optional]
**faulted** | Option<**bool**> | True when this request caused an internal server error and cannot be completed. | [optional][default to false]
**wait_time** | Option<**i32**> | The expected amount to wait (in seconds) to generate all images in this request | [optional]
**queue_position** | Option<**i32**> | The position in the requests queue. This position is determined by relative Kudos amounts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


