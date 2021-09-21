To run the integration tests locally, run:

./.ci/run-tests opensearch
The first argument, opensearch tells the server type to run integration test against. Valid values are opensearch and opendistro. 

Note that integration tests require docker to be installed and running, and downloads quite a bit of data from over the internet and hence take few minutes to complete.