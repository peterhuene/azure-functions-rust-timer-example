# Example Timer-Triggered Azure Function in Rust

This repository is a simple example of a timer-triggered Azure Function written in Rust.

## Building the app

Run `cargo` to build the project:

```text
cargo build --release
```

Copy the target executable to where the Azure Functions host will expect it:

```text
cp target/release/handler .
```

## Running the app locally

To run locally, create a `local.settings.json` file with the following contents:

```json
{
  "IsEncrypted": false,
  "Values": {
    "AzureWebJobsStorage": "UseDevelopmentStorage=true",
    "FUNCTIONS_WORKER_RUNTIME": "custom"
  }
}
```

Start `Azurite` with Docker to locally emulate Azure storage (required for timer-triggered functions):

```text
docker run -p 10000:10000 -p 10001:10001 -p 10002:10002 mcr.microsoft.com/azure-storage/azurite
```

Start the Azure Functions host:

```text
func start
```

The timer-triggered function will execute every 10 seconds. You should see the following output:

```text
[2021-11-11T19:42:40.060Z] Executing 'Functions.TimerExample' (Reason='Timer fired at 2021-11-11T11:42:40.0197830-08:00', Id=36d8ac12-421c-45c7-a7c1-e25fff1c7011)
[2021-11-11T19:42:40.141Z] Invoke request data: InvokeRequest {
[2021-11-11T19:42:40.141Z]     data: {
[2021-11-11T19:42:40.141Z]         "timer": Object({
[2021-11-11T19:42:40.141Z]             "IsPastDue": Bool(
[2021-11-11T19:42:40.141Z]                 false,
[2021-11-11T19:42:40.141Z]             ),
[2021-11-11T19:42:40.141Z]             "Schedule": Object({
[2021-11-11T19:42:40.141Z]                 "AdjustForDST": Bool(
[2021-11-11T19:42:40.141Z]                     true,
[2021-11-11T19:42:40.141Z]                 ),
[2021-11-11T19:42:40.141Z]             }),
[2021-11-11T19:42:40.141Z]             "ScheduleStatus": Null,
[2021-11-11T19:42:40.141Z]         }),
[2021-11-11T19:42:40.141Z]     },
[2021-11-11T19:42:40.141Z]     metadata: {
[2021-11-11T19:42:40.142Z]         "sys": Object({
[2021-11-11T19:42:40.142Z]             "MethodName": String(
[2021-11-11T19:42:40.142Z]                 "TimerExample",
[2021-11-11T19:42:40.142Z]             ),
[2021-11-11T19:42:40.142Z]             "RandGuid": String(
[2021-11-11T19:42:40.142Z]                 "b03493d4-a708-4dd1-a384-d38e56765d43",
[2021-11-11T19:42:40.142Z]             ),
[2021-11-11T19:42:40.142Z]             "UtcNow": String(
[2021-11-11T19:42:40.142Z]                 "2021-11-11T19:42:40.088751Z",
[2021-11-11T19:42:40.142Z]             ),
[2021-11-11T19:42:40.142Z]         }),
[2021-11-11T19:42:40.142Z]     },
[2021-11-11T19:42:40.142Z] }
[2021-11-11T19:42:40.165Z] Executed 'Functions.TimerExample' (Succeeded, Id=36d8ac12-421c-45c7-a7c1-e25fff1c7011, Duration=131ms)
```

## Deploying the app to Azure

Build the app for Linux (with musl libc):

```text
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/handler .
```

Follow the remaining instructions to [publish the project to Azure](https://docs.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Cmacos#publish-the-project-to-azure).
