SELECT transform_hdr.id, transform_type.name, transform_hdr.ref,
(SELECT COUNT(id) FROM transform_line WHERE transform_hdr_id = transform_hdr.id) as "lines"
FROM transform_hdr JOIN transform_type ON transform_hdr.type_id = transform_type.id
ORDER BY id DESC