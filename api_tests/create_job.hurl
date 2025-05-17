# wait for root
PUT {{target}}/api/jobs
[Form]
uri: http://rust-basel.ch/some-job-3.html&query_param=value&whats_this
title: Cool Job
preface: hello 
description: test

HTTP 200
[Captures]
job_id: body



# wait for root
POST {{target}}/api/jobs/{{job_id}}
[Form]
uri: http://rust-basel.ch/some-job-3.html&query_param=value&whats_this
title: Cool Job
preface: hello 
description: test

HTTP 200
[Asserts]

