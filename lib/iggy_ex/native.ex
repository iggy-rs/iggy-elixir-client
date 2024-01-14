defmodule IggyEx.Native do
  @moduledoc false
  use Rustler, otp_app: :iggy_ex, crate: "iggy_nif"

  def admin_connect(), do: :erlang.nif_error(:nif_not_loaded)

  def create_topic(_admin_ref, _topic, _partitions, _replication, _ignore_rack),
    do: :erlang.nif_error(:nif_not_loaded)

  def delete_topic(_admin_ref, _topic), do: :erlang.nif_error(:nif_not_loaded)

  def connect(), do: :erlang.nif_error(:nif_not_loaded)
  def login_user(_iggy_ref), do: :erlang.nif_error(:nif_not_loaded)
end
