SELECT resource.res_id, resource.res_name, resource_type.res_type_name,
(SELECT COUNT(res_qty_id) FROM resource_quantity WHERE resource.res_id = resource_quantity.res_id) as "locations",
(SELECT SUM(qty_val) FROM resource_quantity WHERE resource.res_id = resource_quantity.res_id) as "total quantity"
FROM resource LEFT JOIN resource_type ON resource.res_type_id = resource_type.res_type_id