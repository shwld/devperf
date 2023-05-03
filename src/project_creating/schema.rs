pub enum ProjectAccessToken<T> {
  UseGlobal,
  Override(T),
}
