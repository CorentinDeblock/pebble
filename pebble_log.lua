---@meta

Log = {}

---@param message string
function Log:info(message)
    rust_log:info(message)
end

---@param message string
function Log:error(message) 
    rust_log:error(message)
end

---@param message string
function Log:warn(message) 
    rust_log:warn(message)
end

---@param message string
function Log:debug(message) 
    rust_log:debug(message)
end

---@param message string
function Log:trace(message) 
    rust_log:trace(message)
end