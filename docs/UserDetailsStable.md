# UserDetailsStable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> | The user's unique Username. It is a combination of their chosen alias plus their ID. | [optional]
**id** | Option<**i32**> | The user unique ID. It is always an integer. | [optional]
**kudos** | Option<**f32**> | The amount of Kudos this user has. The amount of Kudos determines the priority when requesting image generations. | [optional]
**evaluating_kudos** | Option<**f32**> | (Privileged) The amount of Evaluating Kudos this untrusted user has from generations and uptime. When this number reaches 50000, they automatically become trusted. | [optional]
**concurrency** | Option<**i32**> | How many concurrent generations this user may request. | [optional]
**worker_invited** | Option<**i32**> | Whether this user has been invited to join a worker to the horde and how many of them. When 0, this user cannot add (new) workers to the horde. | [optional]
**moderator** | Option<**bool**> | This user is a Horde moderator. | [optional]
**kudos_details** | Option<[**crate::models::UserKudosDetails**](UserKudosDetails.md)> |  | [optional]
**worker_count** | Option<**i32**> | How many workers this user has created (active or inactive) | [optional]
**worker_ids** | Option<**Vec<String>**> |  | [optional]
**monthly_kudos** | Option<[**crate::models::MonthlyKudos**](MonthlyKudos.md)> |  | [optional]
**trusted** | Option<**bool**> | This user is a trusted member of the Horde. | [optional]
**suspicious** | Option<**i32**> | (Privileged) How much suspicion this user has accumulated | [optional]
**usage** | Option<[**crate::models::UsageDetailsStable**](UsageDetailsStable.md)> |  | [optional]
**contributions** | Option<[**crate::models::ContributionsDetailsStable**](ContributionsDetailsStable.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


