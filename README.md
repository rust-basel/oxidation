# job-portal-place-holder

## Todo

- find name for the job portal
- change name of the repo


## Proposal

### No Javascript depenendcies just vendored js:

- htmx
- daisy ui
- tailwind
- surreal
- any other js tooling
- web components

### No rust dependencies except:

- http server
- templating
- database access
- maybe uuid or random ID generation
- crypto algorithms
- Test / Dev dependencies


The moment we want to have a login with accounts we set up oauth2 proxy with any idp provider and do not handroll Oauth flows from the get go.

### Components

#### Job Scraper
 - Job that searches for jobs on portals
 - Can be run elsewhere: Maybe Github CI even?

#### Admin portal
 - Create manual entries
 - Accept other kind of entries

#### Actual page 
 - Search
 - Filter
