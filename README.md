# Rust API client for openapi

The API documentation for the Stable Horde


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0
- Package version: 2.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to */api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*V2Api* | [**delete_async_status**](docs/V2Api.md#delete_async_status) | **DELETE** /v2/generate/status/{id} | Cancel an unfinished request
*V2Api* | [**delete_worker_single**](docs/V2Api.md#delete_worker_single) | **DELETE** /v2/workers/{worker_id} | Delete the worker entry
*V2Api* | [**get_async_check**](docs/V2Api.md#get_async_check) | **GET** /v2/generate/check/{id} | Retrieve the status of an Asynchronous generation request without images
*V2Api* | [**get_async_status**](docs/V2Api.md#get_async_status) | **GET** /v2/generate/status/{id} | Retrieve the full status of an Asynchronous generation request
*V2Api* | [**get_find_user**](docs/V2Api.md#get_find_user) | **GET** /v2/find_user | Lookup user details based on their API key
*V2Api* | [**get_horde_load**](docs/V2Api.md#get_horde_load) | **GET** /v2/status/performance | Details about the current performance of this Horde
*V2Api* | [**get_horde_modes**](docs/V2Api.md#get_horde_modes) | **GET** /v2/status/modes | Horde Maintenance Mode Status
*V2Api* | [**get_horde_news**](docs/V2Api.md#get_horde_news) | **GET** /v2/status/news | Read the latest happenings on the horde
*V2Api* | [**get_models**](docs/V2Api.md#get_models) | **GET** /v2/status/models | Returns a list of models active currently in this horde
*V2Api* | [**get_user_single**](docs/V2Api.md#get_user_single) | **GET** /v2/users/{user_id} | Details and statistics about a specific user
*V2Api* | [**get_users**](docs/V2Api.md#get_users) | **GET** /v2/users | A List with the details and statistic of all registered users
*V2Api* | [**get_worker_single**](docs/V2Api.md#get_worker_single) | **GET** /v2/workers/{worker_id} | Details of a registered worker
*V2Api* | [**get_workers**](docs/V2Api.md#get_workers) | **GET** /v2/workers | A List with the details of all registered and active workers
*V2Api* | [**post_async_generate**](docs/V2Api.md#post_async_generate) | **POST** /v2/generate/async | Initiate an Asynchronous request to generate images
*V2Api* | [**post_job_pop**](docs/V2Api.md#post_job_pop) | **POST** /v2/generate/pop | Check if there are generation requests queued for fulfillment
*V2Api* | [**post_job_submit**](docs/V2Api.md#post_job_submit) | **POST** /v2/generate/submit | Submit a generated image
*V2Api* | [**post_sync_generate**](docs/V2Api.md#post_sync_generate) | **POST** /v2/generate/sync | Initiate a Synchronous request to generate images
*V2Api* | [**post_transfer_kudos**](docs/V2Api.md#post_transfer_kudos) | **POST** /v2/kudos/transfer | Transfer Kudos to another registed user
*V2Api* | [**put_horde_modes**](docs/V2Api.md#put_horde_modes) | **PUT** /v2/status/modes | Change Horde Modes
*V2Api* | [**put_user_single**](docs/V2Api.md#put_user_single) | **PUT** /v2/users/{user_id} | Endpoint for horde admins to perform operations on users
*V2Api* | [**put_worker_single**](docs/V2Api.md#put_worker_single) | **PUT** /v2/workers/{worker_id} | Put the worker into maintenance or pause mode


## Documentation For Models

 - [ActiveModel](docs/ActiveModel.md)
 - [ContributionsDetails](docs/ContributionsDetails.md)
 - [ContributionsDetailsStable](docs/ContributionsDetailsStable.md)
 - [ContributionsDetailsStableAllOf](docs/ContributionsDetailsStableAllOf.md)
 - [DeletedWorker](docs/DeletedWorker.md)
 - [Generation](docs/Generation.md)
 - [GenerationInput](docs/GenerationInput.md)
 - [GenerationPayload](docs/GenerationPayload.md)
 - [GenerationStable](docs/GenerationStable.md)
 - [GenerationStableAllOf](docs/GenerationStableAllOf.md)
 - [GenerationSubmitted](docs/GenerationSubmitted.md)
 - [HordeModes](docs/HordeModes.md)
 - [HordePerformance](docs/HordePerformance.md)
 - [HordePerformanceStable](docs/HordePerformanceStable.md)
 - [HordePerformanceStableAllOf](docs/HordePerformanceStableAllOf.md)
 - [KudosTransferred](docs/KudosTransferred.md)
 - [ModelGenerationInputStable](docs/ModelGenerationInputStable.md)
 - [ModelGenerationInputStableAllOf](docs/ModelGenerationInputStableAllOf.md)
 - [ModelPayloadRootStable](docs/ModelPayloadRootStable.md)
 - [ModelPayloadStable](docs/ModelPayloadStable.md)
 - [ModelPayloadStableAllOf](docs/ModelPayloadStableAllOf.md)
 - [ModifyUser](docs/ModifyUser.md)
 - [ModifyWorker](docs/ModifyWorker.md)
 - [MonthlyKudos](docs/MonthlyKudos.md)
 - [Newspiece](docs/Newspiece.md)
 - [NoValidRequestFound](docs/NoValidRequestFound.md)
 - [NoValidRequestFoundStable](docs/NoValidRequestFoundStable.md)
 - [NoValidRequestFoundStableAllOf](docs/NoValidRequestFoundStableAllOf.md)
 - [PopInput](docs/PopInput.md)
 - [PopInputStable](docs/PopInputStable.md)
 - [PopInputStableAllOf](docs/PopInputStableAllOf.md)
 - [PostJobSubmitRequest](docs/PostJobSubmitRequest.md)
 - [PostTransferKudosRequest](docs/PostTransferKudosRequest.md)
 - [PutHordeModesRequest](docs/PutHordeModesRequest.md)
 - [PutUserSingleRequest](docs/PutUserSingleRequest.md)
 - [PutWorkerSingleRequest](docs/PutWorkerSingleRequest.md)
 - [RequestAsync](docs/RequestAsync.md)
 - [RequestError](docs/RequestError.md)
 - [RequestStatusCheck](docs/RequestStatusCheck.md)
 - [RequestStatusStable](docs/RequestStatusStable.md)
 - [RequestStatusStableAllOf](docs/RequestStatusStableAllOf.md)
 - [UsageDetails](docs/UsageDetails.md)
 - [UsageDetailsStable](docs/UsageDetailsStable.md)
 - [UsageDetailsStableAllOf](docs/UsageDetailsStableAllOf.md)
 - [UserDetails](docs/UserDetails.md)
 - [UserDetailsStable](docs/UserDetailsStable.md)
 - [UserDetailsStableAllOf](docs/UserDetailsStableAllOf.md)
 - [UserKudosDetails](docs/UserKudosDetails.md)
 - [WorkerDetails](docs/WorkerDetails.md)
 - [WorkerDetailsStable](docs/WorkerDetailsStable.md)
 - [WorkerDetailsStableAllOf](docs/WorkerDetailsStableAllOf.md)
 - [WorkerKudosDetails](docs/WorkerKudosDetails.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



