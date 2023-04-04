# Authentication

A font-end such as [azure-nextflow-ui](https://github.com/axgonz/azure-nextflow-ui) is required to provide a user the ability to login.

## Overview

The following describes how to use Azure AD to enable authentication between a font-end application and the `azure-nextflow` API.

> Only the steps required to configure the App Registrations are described here; the underlying code pieces can be found in their respective places:
> - [azure-nextflow (API) | auth.rs](../azure/functions/rust/src/services/auth.rs)
> - [azure-nextflow-ui (SPA) | auth.rs](https://github.com/axgonz/azure-nextflow-ui/blob/main/spa/src/controllers/auth.rs)

## App Registrations

Create an app registration for the API and the SPA.

### API
- Name `azure-nextflow`
- Select the `Microsoft only - Single` for *Supported account types*.
- Create the case-sensitive `admin` *App role*.
- Expose an API:
    - Create the case-sensitive `user_impersonation` *Scope*.
    - Add the `azure-nextflow-ui` app registration's `clientId` as an *Authorized client application* (this will have to be updated after the SPA app registration is created).

> Important:
> - Do not add any *platforms*; (remove any existing platforms).
> - Do not create any *certificates* or *secrets*;  (remove any existing certificates or secrets).
> - Do not add any *permissions*; (remove any existing permissions).

### SPA

- Name `azure-nextflow-ui`
- Select the `Microsoft only - Single` for *Supported account types*.
- Add a `Single-page application` *Platform*.
    - Redirect Uri: `https://xxxxxx.xx.web.core.windows.net/login`
    - Logout Uri: `https://xxxxxx.xx.web.core.windows.net/logout`
    - Do not tick any `token` boxes; (untick any if ticked automatically).
    - Do not tick any `implicit flow` boxes; (untick any if ticked automatically).
- Add the `User.Read` *API permission*.
- Add the `user_impersonation` *API permission* that was exposed by API app registration.

> Important:
> - Do not create any *certificates* or *secrets*;  (remove any existing certificates or secrets).
> - Do not add any *app roles*; (remove any existing app roles).
> - Do to add any *Scopes*; (remove any existing scopes).

## Azure Functions

Enable Authentication by adding the existing `azure-nextflow` app registration to the Azure function.

- Provide `https://sts.windows.net/72f988bf-86f1-41af-91ab-2d7cd011db47/v2.0` as the *Issuer URL*.
- Clear the *Client secret settings name*; this needs to be empty.
- Add the Application ID URI `api://xxxxxxxx-xxxx-xxxx-xxxxxxxxxxxx` in *Allowed token audiences* (this value can be found under *Expose An API* on the `azure-nextflow` app registraion).
- Select `HTTP 401 Unauthorized` for *Unauthenticated requests*.


## User Role Assignment

Provide the desired users with access to the application.

- Find `azure-nextflow` registration under *Enterprise applications* (not under *App registrations* this time).
- Add each user to the `admin` role under *Users and groups*.