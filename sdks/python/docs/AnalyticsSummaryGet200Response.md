# AnalyticsSummaryGet200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_views** | **int** |  | [optional] 
**unique_visitors** | **int** |  | [optional] 

## Example

```python
from freeradical_client.models.analytics_summary_get200_response import AnalyticsSummaryGet200Response

# TODO update the JSON string below
json = "{}"
# create an instance of AnalyticsSummaryGet200Response from a JSON string
analytics_summary_get200_response_instance = AnalyticsSummaryGet200Response.from_json(json)
# print the JSON string representation of the object
print(AnalyticsSummaryGet200Response.to_json())

# convert the object into a dict
analytics_summary_get200_response_dict = analytics_summary_get200_response_instance.to_dict()
# create an instance of AnalyticsSummaryGet200Response from a dict
analytics_summary_get200_response_from_dict = AnalyticsSummaryGet200Response.from_dict(analytics_summary_get200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


