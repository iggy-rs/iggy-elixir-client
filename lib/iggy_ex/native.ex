defmodule IggyEx.Native do
  @moduledoc false
  use Rustler, otp_app: :iggy_ex, crate: "iggy_nif"

  def ping(), do: :erlang.nif_error(:nif_not_loaded)
  def login_user(_iggy_ref), do: :erlang.nif_error(:nif_not_loaded)
end
