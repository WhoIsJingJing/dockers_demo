
import flatten from require "pgmoon.util"

class CqueuesSocket
  connect: (host, port, opts) =>
    socket = require "cqueues.socket"
    errno = require "cqueues.errno"

    @sock = socket.connect {
      :host
      :port
    }

    if @timeout
      @sock\settimeout @timeout

    @sock\setmode "bn", "bn"
    success, err =  @sock\connect!
    unless success
      return nil, errno.strerror(err)

    true

  sslhandshake: =>
    @sock\starttls!

  send: (...) =>
    @sock\write flatten ...

  receive: (...) =>
    @sock\read ...

  close: =>
    @sock\close!

  settimeout: (t) =>
    if t
      t = t/1000

    if @sock
      @sock\settimeout t
    else
      @timeout = t

  -- openresty pooling interface, disable pooling
  getreusedtimes: => 0

{ :CqueuesSocket }


