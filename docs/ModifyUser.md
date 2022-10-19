# ModifyUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**new_kudos** | Option<**f32**> | The new total Kudos this user has after this request | [optional]
**concurrency** | Option<**i32**> | The request concurrency this user has after this request | [optional]
**usage_multiplier** | Option<**f32**> | Multiplies the amount of kudos lost when generating images. | [optional]
**worker_invited** | Option<**i32**> | Whether this user has been invited to join a worker to the horde and how many of them. When 0, this user cannot add (new) workers to the horde. | [optional]
**moderator** | Option<**bool**> | The user's new moderator status. | [optional]
**public_workers** | Option<**bool**> | The user's new public_workers status. | [optional]
**username** | Option<**String**> | The user's new username. | [optional]
**monthly_kudos** | Option<**i32**> | The user's new monthly kudos total | [optional]
**trusted** | Option<**bool**> | The user's new trusted status | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


