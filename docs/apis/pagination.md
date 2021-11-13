# Pagination

## 场景

```javascript
const state = {
  current_cursor_info: {
    start: null,
    current: null,
    end: null,
  },
  skip: [],
};
```

1. 首次请求`/post/posts`
2. 缓存当前最前`cursor`和用户当前正在浏览的`cursor`

   ```javascript
   state.current_cursor_info.start = start_cursor;
   state.current_cursor_info.current = start_cursor;
   state.current_cursor_info.end = end_cursor;
   ```

3. 本页的数据用完之后,请求`/post/posts?after=:end_cursor&skip[]=skip[]`

   > 注意，这里的 skip 信息为历史 skip 数据，不必包括当前活跃的 skip info

4. 当用户从后台，再次进入的时候：

   把`current_cursor_info`的信息，push 到`skip`中：

   ```javascript
   state.skip.push({
     start: state.current_cursor_info.start,
     end: state.current_cursor_info.current,
   });
   ```

5. 请求 `/post/posts?skip[]=skip[]`

6. 缓存当前最前`cursor`和用户当前正在浏览的`cursor`

   ```javascript
   state.current_cursor_info.start = start_cursor;
   state.current_cursor_info.current = start_cursor;
   state.current_cursor_info.end = end_cursor;
   ```

7. 本页的数据用完之后,请求`/post/posts?after=:end_cursor&skip[]=skip[]`
8. 当请求返回没有的时候,就可以用之前的`/post/posts?after=:last_cursor`继续请求
