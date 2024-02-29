## Installing k6

```bash
brew install k6
```

## Building and Running the App

In a dedicated terminal session, run the following commands:

```bash
pushd scenario
spin b
spin u
```

## Test Web Dashboard

While tests are running, you can access tests result at [http://127.0.0.1:5665](http://127.0.0.1:5665).

## Running Smoke Test

```bash
K6_WEB_DASHBOARD=true k6 run smoke-test.js
```

## Running Stress Test

```bash
K6_WEB_DASHBOARD=true k6 run stress-test.js
```

## Running Breakpoint Test

```bash
K6_WEB_DASHBOARD=true k6 run breakpoint-test.js
```