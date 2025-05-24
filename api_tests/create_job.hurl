# wait for root
PUT {{target}}/jobs
[FormParams]
uri: http://rust-basel.ch/some-job-3.html&query_param=value&whats_this
title: Cool Job
preface: hello 
description: test

HTTP 200
[Captures]
job_id: body



# update a job
POST {{target}}/api/jobs/{{job_id}}
[FormParams]
uri: http://rust-basel.ch/some-job-3.html&query_param=value&whats_this
title: Cool Job
preface: hello 
description: test

HTTP 200
[Asserts]

