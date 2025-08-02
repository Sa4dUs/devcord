# Message Service API Specification

---

## 1. WebSocket Messaging

**GET** `/`

**Headers:**

-   `Authorization: Bearer <JWT>`

**WebSocket Upgrade:**

-   Protocols: JWT is sent as a subprotocol to validate the user.

**Behavior:**

-   Establishes a WebSocket connection for real-time messaging.
-   Only available if the user is a member of the group associated with the `channel_id`.
-   Automatically fetches the group associated with the channel using the Group Service.
-   Sends received messages to all group members via Kafka.
-   Persists the message to the database.

**Message Format:**

-   Each incoming message should be plain text.
-   Each message is persisted with the sender ID, channel ID, and timestamp.

**Error Handling:**

-   If the user is not authorized or not part of the group, the socket is closed silently.
-   If group/member lookup fails, the connection is not established.
-   Server logs errors, but clients are not given detailed error responses over WebSocket.

---

## 2. Fetch Channel Messages

**GET** `/{channel_id}`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Path Parameters:**

-   `channel_id` (UUID): The channel to fetch messages from.

**Query Parameters:**

-   `from` (optional, integer): Start index (inclusive, default: 0).
-   `to` (optional, integer): End index (exclusive, default: 10).

**Responses:**

-   `200 OK`

    ```json
    [
    	{
    		"id": "uuid",
    		"sender_id": "uuid",
    		"channel_id": "uuid",
    		"message": "Hello",
    		"created_at": "2025-08-01T10:11:12"
    	}
    ]
    ```

-   `401 Unauthorized` — If JWT is missing or invalid.
-   `404 Not Found` — If the requesting user is not a member of the channel's group.
-   `500 Internal Server Error` — On invalid UUID, group member fetch failure, or database errors.

---

## Internal Dependencies & Middleware

### Middleware Used

-   `require_auth`: Extracts and validates the JWT. Injects user claims as `Authenticated`.
-   `extract_channel`: Extracts `channel_id` from the request path and injects it as `Channel`.

---

## External Service Calls

The message service depends on the **Group Service** to:

1. **Get Group by Channel ID**

    **GET** `/by_channel/{channel_id}`  
    Used to fetch the group ID associated with a given `channel_id`.

2. **Get Group Members**

    **GET** `/{group_id}/members`  
    Used to fetch the list of users allowed to access and receive messages from the group.

Both requests include:

-   Header: `Authorization: Bearer <JWT>`

---
