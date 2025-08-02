# Group Service API Specification

---

## 1. Create Group

**POST** `/create`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Body:**

```json
{
	"member_ids": ["uuid-1", "uuid-2"]
}
```

**Responses:**

-   `201 Created`
    ```json
    "uuid-of-created-group"
    ```
-   `400 Bad Request` — If `member_ids` is empty.
-   `401 Unauthorized` — If JWT is missing or invalid.
-   `500 Internal Server Error` — On database or server error.

---

## 2. Add Users to Group

**PUT** `/{group_id}/add-users`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Path Parameters:**

-   `group_id` (UUID): The group to add users to.

**Body:**

```json
{
	"user_ids": ["uuid-3", "uuid-4"]
}
```

**Responses:**

-   `204 No Content` — Users added successfully.
-   `401 Unauthorized` — If JWT is missing or invalid.
-   `403 Forbidden` — If requester is not the group owner.
-   `404 Not Found` — If group does not exist.
-   `500 Internal Server Error` — On database or server error.

---

## 3. Remove User from Group

**POST** `/{group_id}/remove-user`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Path Parameters:**

-   `group_id` (UUID): The group to remove a user from.

**Body:**

```json
{
	"user_id": "uuid-to-remove"
}
```

**Responses:**

-   `204 No Content` — User removed successfully.
-   `401 Unauthorized` — If JWT is missing or invalid.
-   `403 Forbidden` — If requester is not the group owner or tries to remove themselves.
-   `404 Not Found` — If group does not exist.
-   `500 Internal Server Error` — On database or server error.

---

## 4. Delete Group

**DELETE** `/{group_id}`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Path Parameters:**

-   `group_id` (UUID): The group to delete.

**Responses:**

-   `204 No Content` — Group deleted successfully.
-   `401 Unauthorized` — If JWT is missing or invalid.
-   `403 Forbidden` — If requester is not the group owner.
-   `404 Not Found` — If group does not exist.
-   `500 Internal Server Error` — On database or server error.

---

## 5. List User Groups

**GET** `/user-groups`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Query Parameters:**

-   `from` (optional, integer): Start index (inclusive, default: 0).
-   `to` (optional, integer): End index (exclusive, default: 10).

**Responses:**

-   `200 OK`
    ```json
    [
    	{
    		"id": "uuid",
    		"owner_id": "uuid",
    		"channel_id": "uuid",
    		"created_at": "2025-07-29T12:34:56"
    	}
    ]
    ```
-   `401 Unauthorized` — If JWT is missing or invalid.
-   `500 Internal Server Error` — On database or server error.

---

**General Notes:**

-   All endpoints require a valid JWT in the `Authorization` header.
-   All UUIDs must be valid.
-   All error responses return a plain message or empty body, depending on the status

---

## 6. Get Group Members

**GET** `/{group_id}/members`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Path Parameters:**

-   `group_id` (UUID): The group to retrieve members from.

**Responses:**

-   `200 OK`

    ```json
    ["uuid-1", "uuid-2", "uuid-3"]
    ```

-   `401 Unauthorized` — If JWT is missing or invalid.
-   `404 Not Found` — If group does not exist or requester is not a member.
-   `500 Internal Server Error` — On database or server error.

---

## 7. Get Group ID by Channel ID

**GET** `/by_channel/{channel_id}`

**Headers:**

-   `Authorization: Bearer <JWT>`

**Path Parameters:**

-   `channel_id` (UUID): The channel to retrieve the associated group ID from.

**Responses:**

-   `200 OK`

    ```json
    ["uuid-1", "uuid-2", "uuid-3"]
    ```

-   `401 Unauthorized` — If JWT is missing or invalid.
-   `404 Not Found` — If no group is associated with the given channel ID.
-   `500 Internal Server Error` — On database or server error.
