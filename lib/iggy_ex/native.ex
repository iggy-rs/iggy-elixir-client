defmodule IggyEx do
  @moduledoc false
  use Rustler, otp_app: :iggy_ex, crate: "iggy_nif"

  def connect(), do: :erlang.nif_error(:nif_not_loaded)
  def ping(), do: :erlang.nif_error(:nif_not_loaded)
  def login_user(_username, _password), do: :erlang.nif_error(:nif_not_loaded)
  def create_stream(_stream_id, _name), do: :erlang.nif_error(:nif_not_loaded)
end
