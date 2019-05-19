SELECT transform_hdr.transform_hdr_id, transform_type.transf_type_name, transform_hdr.transform_ref,
(SELECT COUNT(transform_line_id) FROM transform_line WHERE transform_hdr_id = transform_hdr.transform_hdr_id) as "lines"
FROM transform_hdr JOIN transform_type ON transform_hdr.transform_type_id = transform_type.transf_type_id
ORDER BY transform_hdr_id DESC LIMIT 30