# NoValidRequestFoundStable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**worker_id** | Option<**i32**> | How many waiting requests were skipped because they demanded a specific worker | [optional]
**performance** | Option<**i32**> | How many waiting requests were skipped because they demanded a specific worker | [optional]
**nsfw** | Option<**i32**> | How many waiting requests were skipped because they demanded a nsfw generation which this worker does not provide. | [optional]
**blacklist** | Option<**i32**> | How many waiting requests were skipped because they demanded a generation with a word that this worker does not accept. | [optional]
**untrusted** | Option<**i32**> | How many waiting requests were skipped because they demanded a trusted worker which this worker is not. | [optional]
**models** | Option<**i32**> | How many waiting requests were skipped because they demanded a different model than what this worker provides. | [optional]
**max_pixels** | Option<**i32**> | How many waiting requests were skipped because they demanded a higher size than this worker provides | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


