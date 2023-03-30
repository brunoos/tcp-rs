local core = require("net.tcp.core")

local function select(readers, writers, excepts)
  local rfd, wfd, efd
  
  if readers then
    rfd = {}
    for i, sock in ipairs(readers) do
      rfd[i] = sock:_getfd()
    end
  end
    
  if writers then
    wfd = {}
    for i, sock in ipairs(writers) do
      wfd[i] = sock:_getfd()
    end
  end

  if excepts then
    efd = {}
    for i, sock in ipairs(excepts) do
      efd[i] = sock:_getfd()
    end
  end

  if not core.select(rfd, wfd, efd) then
    return false
  end

  local rres, wres, eres

  if readers then
    rres = {}
    for i, active in ipairs(rfd) do
      if active then 
        table.insert(rres, readers[i])
      end
    end
  end

  if writers then
    wres = {}
    for i, active in ipairs(wfd) do
      if active then 
        table.insert(wres, writers[i])
      end
    end
  end

  if excepts then
    eres = {}
    for i, active in ipairs(efd) do
      if active then 
        table.insert(eres, excepts[i])
      end
    end
  end

  return true, rres, wres, eres
end

return {
  connect = core.connect,
  select  = select,
}
