# freeradical_client.DefaultApi

All URIs are relative to *http://localhost:8000/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analytics_summary_get**](DefaultApi.md#analytics_summary_get) | **GET** /analytics/summary | Get analytics summary
[**auth_login_post**](DefaultApi.md#auth_login_post) | **POST** /auth/login | Authenticate user
[**pages_get**](DefaultApi.md#pages_get) | **GET** /pages | List pages
[**pages_post**](DefaultApi.md#pages_post) | **POST** /pages | Create a page
[**pages_uuid_get**](DefaultApi.md#pages_uuid_get) | **GET** /pages/{uuid} | Get page by UUID


# **analytics_summary_get**
> AnalyticsSummaryGet200Response analytics_summary_get()

Get analytics summary

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import freeradical_client
from freeradical_client.models.analytics_summary_get200_response import AnalyticsSummaryGet200Response
from freeradical_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8000/v1
# See configuration.py for a list of all supported configuration parameters.
configuration = freeradical_client.Configuration(
    host = "http://localhost:8000/v1"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure Bearer authorization (JWT): bearerAuth
configuration = freeradical_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with freeradical_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = freeradical_client.DefaultApi(api_client)

    try:
        # Get analytics summary
        api_response = api_instance.analytics_summary_get()
        print("The response of DefaultApi->analytics_summary_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->analytics_summary_get: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**AnalyticsSummaryGet200Response**](AnalyticsSummaryGet200Response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Analytics data |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **auth_login_post**
> LoginResponse auth_login_post(login_request)

Authenticate user

### Example


```python
import freeradical_client
from freeradical_client.models.login_request import LoginRequest
from freeradical_client.models.login_response import LoginResponse
from freeradical_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8000/v1
# See configuration.py for a list of all supported configuration parameters.
configuration = freeradical_client.Configuration(
    host = "http://localhost:8000/v1"
)


# Enter a context with an instance of the API client
with freeradical_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = freeradical_client.DefaultApi(api_client)
    login_request = freeradical_client.LoginRequest() # LoginRequest | 

    try:
        # Authenticate user
        api_response = api_instance.auth_login_post(login_request)
        print("The response of DefaultApi->auth_login_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->auth_login_post: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **login_request** | [**LoginRequest**](LoginRequest.md)|  | 

### Return type

[**LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful login |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pages_get**
> List[Page] pages_get(page=page, limit=limit)

List pages

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import freeradical_client
from freeradical_client.models.page import Page
from freeradical_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8000/v1
# See configuration.py for a list of all supported configuration parameters.
configuration = freeradical_client.Configuration(
    host = "http://localhost:8000/v1"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure Bearer authorization (JWT): bearerAuth
configuration = freeradical_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with freeradical_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = freeradical_client.DefaultApi(api_client)
    page = 56 # int |  (optional)
    limit = 56 # int |  (optional)

    try:
        # List pages
        api_response = api_instance.pages_get(page=page, limit=limit)
        print("The response of DefaultApi->pages_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->pages_get: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **int**|  | [optional] 
 **limit** | **int**|  | [optional] 

### Return type

[**List[Page]**](Page.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List of pages |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pages_post**
> Page pages_post(page=page)

Create a page

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import freeradical_client
from freeradical_client.models.page import Page
from freeradical_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8000/v1
# See configuration.py for a list of all supported configuration parameters.
configuration = freeradical_client.Configuration(
    host = "http://localhost:8000/v1"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure Bearer authorization (JWT): bearerAuth
configuration = freeradical_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with freeradical_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = freeradical_client.DefaultApi(api_client)
    page = freeradical_client.Page() # Page |  (optional)

    try:
        # Create a page
        api_response = api_instance.pages_post(page=page)
        print("The response of DefaultApi->pages_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->pages_post: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | [**Page**](Page.md)|  | [optional] 

### Return type

[**Page**](Page.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Page created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pages_uuid_get**
> Page pages_uuid_get(uuid)

Get page by UUID

### Example

* Bearer (JWT) Authentication (bearerAuth):

```python
import freeradical_client
from freeradical_client.models.page import Page
from freeradical_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:8000/v1
# See configuration.py for a list of all supported configuration parameters.
configuration = freeradical_client.Configuration(
    host = "http://localhost:8000/v1"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure Bearer authorization (JWT): bearerAuth
configuration = freeradical_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with freeradical_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = freeradical_client.DefaultApi(api_client)
    uuid = 'uuid_example' # str | 

    try:
        # Get page by UUID
        api_response = api_instance.pages_uuid_get(uuid)
        print("The response of DefaultApi->pages_uuid_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->pages_uuid_get: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **uuid** | **str**|  | 

### Return type

[**Page**](Page.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Page details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

