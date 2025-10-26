import { createSlice, configureStore } from "@reduxjs/toolkit";

export const mapSlice = createSlice({
  name: "active_marker",
  initialState: {
    value: "",
  },
  reducers: {
    setActiveMarker: (state, action) => {
      state.value = action.payload;
    },
  },
});

// Action creators are generated for each case reducer function
export const { setActiveMarker } = mapSlice.actions;

const store = configureStore({
  reducer: {
    map: mapSlice.reducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
