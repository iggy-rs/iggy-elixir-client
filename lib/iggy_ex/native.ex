defmodule IggyEx do
  @moduledoc false
  use Rustler, otp_app: :iggy_ex, crate: "iggy_nif"

  def ping(), do: :erlang.nif_error(:nif_not_loaded)
  def login_user(_username, _password), do: :erlang.nif_error(:nif_not_loaded)
end
