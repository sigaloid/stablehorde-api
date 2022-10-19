# GenerationInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prompt** | **String** | The prompt which will be sent to Stable Diffusion to generate an image | 
**params** | Option<[**crate::models::ModelGenerationInputStable**](ModelGenerationInputStable.md)> |  | [optional]
**nsfw** | Option<**bool**> | Set to true if this request is NSFW. This will skip workers which censor images. | [optional][default to false]
**trusted_workers** | Option<**bool**> | When true, only trusted workers will serve this request. When False, Evaluating workers will also be used which can increase speed but adds more risk! | [optional][default to true]
**censor_nsfw** | Option<**bool**> | If the request is SFW, and the worker accidentaly generates NSFW, it will send back a censored image. | [optional][default to false]
**workers** | Option<**Vec<String>**> |  | [optional]
**models** | Option<**Vec<String>**> |  | [optional]
**source_image** | Option<**String**> | The Base64-encoded webp to use for img2img | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


